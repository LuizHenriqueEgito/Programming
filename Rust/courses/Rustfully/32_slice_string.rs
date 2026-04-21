fn main() {
    let sencente = String::from("Bob loves Chinese food.");
    let name = &sentence[0..3];  // poderiamos usar [..3]
    let food = &sencente[10..17];  // isso faz o slice da sting
    // funciona também [18..]; -> pega food.
}