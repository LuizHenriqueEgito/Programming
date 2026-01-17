// &str é imutavel
fn main() {
    let wisdom = "Wash your hands with soap.";  // mesmo colocando let mut não dá para mudar, esse valor está na static memory
    println!("wisdom: {wisdom}");
    let mut s: String = String::from("Esse é mutavel");  // Esse valor vai para o HEAP e podemos modifica-lo
    println!("s: {s}");
    s += " Adicionando novo texto";
    println!("s: {s}");
}
