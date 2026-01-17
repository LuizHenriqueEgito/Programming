// é preciso intalar use no terminal `cargo add rand`
use rand::Rng;  // use faz o import
use std::io;
use std::cmp::Ordering;

// TODO: Adicione a funcionalidade para falar quantas tentativas foram necessarias para acertar
fn main() {
    println!("Pense em um número!");
    let secret_number: u32 = rand::rng().random_range(1..=100);
    // println!("O número pensado é {secret_number}");

    loop {
        println!("Escreva seu palpite:");
        let mut guess: String = String::new()
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");
        println!("Seu palpite foi {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Você chutou corretamente!");
                break;
            }
        }

    }
}