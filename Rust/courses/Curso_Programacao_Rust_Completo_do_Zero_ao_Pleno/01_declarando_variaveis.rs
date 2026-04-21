fn main() {
    let name: &str = "Luiz";
    let mut age = 42;
    age += 1; // não é possivel fazer age++ como no GO ou no C
    println!("Hello {}! Age: {}", name, age); // variaveis sem mut são imutaveis no rust
}