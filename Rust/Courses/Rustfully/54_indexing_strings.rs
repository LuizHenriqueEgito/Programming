fn main() {
    let name: String = String::from("Bob");
    let lenght: usize = name.len(); // comprimento em bytes!
    println!("name={name}, lenght={lenght}");
    // você precisa ficar atento entre pegar os caracteres ou os bytes! 
}