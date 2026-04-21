use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Advinhe o número!");
    println!("Digite o seu palpite.");

    // gerando o número aleatório
    let secret_number: u32 = rand::rng().random_range(1..=100);
    // println!("O número aleatório gerado foi: {}", secret_number);
    loop {
        // iniciando o chute do jogador
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a entrada");

        println!("Seu palpite: {}", guess);
        let guess: u32 = guess
            .trim()
            .parse() {
                Ok(num: u32) => num,
                Err(_) => continue
            }
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!!");
                break;
            }
        }
    }
}