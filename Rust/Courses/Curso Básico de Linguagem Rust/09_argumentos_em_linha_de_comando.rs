// rode assim: ./09_argumentos_em_linha_de_comando abc def ghi
use std::env;

fn main() {
    println!("\nTotal de elementos em env::args é {}", env::args().len());
    println!("\nPercorre usando iterador:");
    let mut i: i32 = 0;
    for x in env::args() {
        println!("Argumento [{}] == {}", i, x);
        i += 1;
    }

    println!("\nPercorre usando iterador com indices:");
    for (i, x) in env::args().enumerate() {
        println!("Argumento [{}] == {}", i, x);
    }

    println!("\nColoca tudo em um vetor");
    let argumentos: Vec<String> = env::args().collect();

    println!("\nPercorre usando o vetor:");
    for i in 0..argumentos.len() {
        println!("Argumento [{}] == {}", i, argumentos[i]);
    }
    println!("Total de {} elementos no vetor", argumentos.len())
}