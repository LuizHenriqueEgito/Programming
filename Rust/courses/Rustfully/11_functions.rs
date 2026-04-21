
// a função pode vir antes da função main
fn print_hello() {
    println!("Hello, Bob!");
}

fn main() {
    print_hello();
    print_hello();
    print_hello();
    goodbye();
}

// a função pode vir depois da função main
fn goodbye() {
    println!("Goodbye, Bob!");
}