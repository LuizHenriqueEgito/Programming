/*
Funções passam o ownership se forem passadas diretas
mas podemos emprestar valores com referencias.
*/

fn get_lenght(text: String) -> usize {
    text.chars().count()
}

fn main() {
    let name = String::from("Bob");
    let len = get_lenght(name);  // isso é uma referencia ao valor você não passa o ownership
    // println!(name);  // isso não funciona
}