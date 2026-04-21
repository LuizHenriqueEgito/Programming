/*
Quando colocamos algo no HEAP duas coisas acontecem:
1. Ele solicita a memória necessaria
2. Ele devolve a memória quando não é mais necessária
*/
fn main() {
    {
        let text: String = String::from("Bob");
    }
    // não posso printar text aqui pois ele já foi destruido e sua memória foi devolvida
    let s1: String = String::from("ABC");
    let s2 = s1;  // Aqui eu destruo s1 e s2 pega o ownership de "A"
    println!("s1 não existe mais mas s2={s2}");
}