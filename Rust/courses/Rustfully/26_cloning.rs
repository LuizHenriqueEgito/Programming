fn main() {
    let mut var = String::from("Bob");
    let var_copy = var.clone();  // implementa o clone

    println!("Hello, {var}");
    println!("Hello, {var_copy}");
}